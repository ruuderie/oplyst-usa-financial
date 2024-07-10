import type { Meta, StoryObj } from '@storybook/vue3';

import SelectLabel from '../components/ui/select/SelectLabel.vue';

const meta = {
  title: 'SelectLabel',
  component: SelectLabel,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof SelectLabel>;

export default meta;
type Story = StoryObj<typeof SelectLabel>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};