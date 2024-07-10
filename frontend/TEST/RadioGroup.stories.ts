import type { Meta, StoryObj } from '@storybook/vue3';

import RadioGroup from '../components/ui/radio-group/RadioGroup.vue';

const meta = {
  title: 'RadioGroup',
  component: RadioGroup,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof RadioGroup>;

export default meta;
type Story = StoryObj<typeof RadioGroup>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};