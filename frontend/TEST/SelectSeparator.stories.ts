import type { Meta, StoryObj } from '@storybook/vue3';

import SelectSeparator from '../components/ui/select/SelectSeparator.vue';

const meta = {
  title: 'SelectSeparator',
  component: SelectSeparator,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof SelectSeparator>;

export default meta;
type Story = StoryObj<typeof SelectSeparator>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};