import type { Meta, StoryObj } from '@storybook/vue3';

import SelectValue from '../components/ui/select/SelectValue.vue';

const meta = {
  title: 'SelectValue',
  component: SelectValue,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof SelectValue>;

export default meta;
type Story = StoryObj<typeof SelectValue>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};